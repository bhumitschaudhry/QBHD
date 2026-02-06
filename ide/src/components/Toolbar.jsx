import { invoke } from '@tauri-apps/api/tauri';

export default function Toolbar({ currentFile, code, setOutput }) {
  const handleCompile = async () => {
    if (!currentFile) return;
    try {
      const result = await invoke('compile_file', { path: currentFile });
      setOutput(result);
    } catch (e) {
      setOutput(`Error: ${e}`);
    }
  };

  const handleCheck = async () => {
    if (!currentFile) return;
    try {
      const result = await invoke('check_file', { path: currentFile });
      setOutput(result);
    } catch (e) {
      setOutput(`Error: ${e}`);
    }
  };

  const handleRun = async () => {
    if (!currentFile) return;
    try {
      await invoke('run_file', { path: currentFile.replace('.bas', '') });
    } catch (e) {
      setOutput(`Error: ${e}`);
    }
  };

  return (
    <div className="toolbar">
      <button onClick={handleCheck}>✓ Check</button>
      <button onClick={handleCompile}>🔨 Build</button>
      <button onClick={handleRun}>▶ Run</button>
    </div>
  );
}
