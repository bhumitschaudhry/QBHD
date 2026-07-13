import { invoke } from '@tauri-apps/api/tauri';

/** Guard: returns true if no file is open (prints message to output). */
function requireFile(currentFile, setOutput) {
  if (!currentFile) {
    setOutput(prev => prev + '\nNo file open. Open a .bas file first.\n');
    return true;
  }
  return false;
}

/** Run a Tauri IPC command, appending output and handling errors. */
async function runCommand(cmd, args, setOutput, prefix = '') {
  try {
    const result = await invoke(cmd, args);
    setOutput(prev => prev + prefix + result + '\n');
    return result;
  } catch (e) {
    setOutput(prev => prev + `\n${cmd} failed:\n${e}\n`);
    return null;
  }
}

export default function Toolbar({ currentFile, setOutput, onSave, modified }) {
  const handleCompile = async () => {
    if (requireFile(currentFile, setOutput)) return;
    setOutput(prev => prev + `\nBuilding ${currentFile}...\n`);
    await runCommand('compile_file', { path: currentFile }, setOutput);
  };

  const handleCheck = async () => {
    if (requireFile(currentFile, setOutput)) return;
    setOutput(prev => prev + `\nChecking ${currentFile}...\n`);
    const result = await runCommand('check_file', { path: currentFile }, setOutput);
    if (result === null) return;
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
      // Non-JSON output already printed by runCommand
    }
  };

  const handleRun = async () => {
    if (requireFile(currentFile, setOutput)) return;

    // Save first if modified
    if (modified && onSave) {
      await onSave();
    }

    setOutput(prev => prev + `\nBuilding and running...\n`);
    const compileResult = await runCommand('compile_file', { path: currentFile }, setOutput);
    if (compileResult === null) return;

    const runPath = currentFile.replace(/\.bas$/i, '');
    await runCommand('run_file', { path: runPath }, setOutput);
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
