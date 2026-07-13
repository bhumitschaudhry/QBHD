import { invoke } from '@tauri-apps/api/tauri';

export default function Toolbar({ currentFile, code, setOutput, onSave, modified }) {
  const handleCompile = async () => {
    if (!currentFile) {
      setOutput(prev => prev + '\nNo file open. Open a .bas file first.\n');
      return;
    }
    setOutput(prev => prev + `\nBuilding ${currentFile}...\n`);
    try {
      const result = await invoke('compile_file', { path: currentFile });
      setOutput(prev => prev + result + '\n');
    } catch (e) {
      setOutput(prev => prev + `\nBuild failed:\n${e}\n`);
    }
  };

  const handleCheck = async () => {
    if (!currentFile) {
      setOutput(prev => prev + '\nNo file open. Open a .bas file first.\n');
      return;
    }
    setOutput(prev => prev + `\nChecking ${currentFile}...\n`);
    try {
      const result = await invoke('check_file', { path: currentFile });
      try {
        const diags = JSON.parse(result);
        if (diags.length === 0) {
          setOutput(prev => prev + 'No issues found.\n');
        } else {
          const formatted = diags.map(d =>
            `  ${d.severity}: ${d.message} (line ${d.line}, col ${d.column})`
          ).join('\n');
          setOutput(prev => prev + `Found ${diags.length} issue(s):\n${formatted}\n`);
        }
      } catch {
        setOutput(prev => prev + result + '\n');
      }
    } catch (e) {
      setOutput(prev => prev + `\nCheck failed:\n${e}\n`);
    }
  };

  const handleRun = async () => {
    if (!currentFile) {
      setOutput(prev => prev + '\nNo file open. Open a .bas file first.\n');
      return;
    }

    // Save first if modified
    if (modified && onSave) {
      await onSave();
    }

    setOutput(prev => prev + `\nBuilding and running...\n`);
    try {
      // Compile first
      const compileResult = await invoke('compile_file', { path: currentFile });
      setOutput(prev => prev + compileResult + '\n');

      // Run the compiled binary
      const runPath = currentFile.replace(/\.bas$/i, '');
      const result = await invoke('run_file', { path: runPath });
      setOutput(prev => prev + result + '\n');
    } catch (e) {
      setOutput(prev => prev + `\nRun failed:\n${e}\n`);
    }
  };

  const handleNewFile = async () => {
    try {
      const { save } = await import('@tauri-apps/api/dialog');
      const filePath = await save({
        filters: [{ name: 'BASIC', extensions: ['bas'] }],
        title: 'New BASIC File',
      });
      if (filePath) {
        const defaultCode = "' QBHD BASIC Program\n' Created with QBHD IDE\n\nPRINT \"Hello, World!\"\n";
        await invoke('save_file', { path: filePath, contents: defaultCode });
        // Trigger file open
        const name = filePath.split(/[/\\]/).pop();
        window.dispatchEvent(new CustomEvent('qbhd-open-file', {
          detail: { path: filePath, name }
        }));
      }
    } catch (e) {
      setOutput(prev => prev + `\nError creating file: ${e}\n`);
    }
  };

  return (
    <div className="toolbar">
      <div className="toolbar-group">
        <button className="toolbar-btn" onClick={handleNewFile} title="New File (Ctrl+N)">
          📄 New
        </button>
        <button
          className={`toolbar-btn ${modified ? 'modified' : ''}`}
          onClick={onSave}
          title="Save (Ctrl+S)"
          disabled={!currentFile}
        >
          💾 Save{modified ? ' •' : ''}
        </button>
      </div>
      <div className="toolbar-separator" />
      <div className="toolbar-group">
        <button className="toolbar-btn" onClick={handleCheck} title="Check for errors" disabled={!currentFile}>
          ✓ Check
        </button>
        <button className="toolbar-btn primary" onClick={handleCompile} title="Build program" disabled={!currentFile}>
          🔨 Build
        </button>
        <button className="toolbar-btn success" onClick={handleRun} title="Build and Run" disabled={!currentFile}>
          ▶ Run
        </button>
      </div>
      <div className="toolbar-spacer" />
      <div className="toolbar-group">
        {currentFile && (
          <span className="toolbar-file-info">
            {currentFile.split(/[/\\]/).pop()}
          </span>
        )}
      </div>
    </div>
  );
}
