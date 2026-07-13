import { useState, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Editor from './components/Editor';
import FileTree from './components/FileTree';
import Terminal from './components/Terminal';
import Toolbar from './components/Toolbar';
import StatusBar from './components/StatusBar';
import './App.css';

function App() {
  const [currentFile, setCurrentFile] = useState(null);
  const [fileName, setFileName] = useState('');
  const [code, setCode] = useState('');
  const [output, setOutput] = useState('Welcome to QBHD IDE\n');
  const [modified, setModified] = useState(false);
  const [cursorPos, setCursorPos] = useState({ line: 1, column: 1 });
  const [projectDir, setProjectDir] = useState('.');

  // Load file content when a file is selected
  const handleFileSelect = useCallback(async (path, name) => {
    try {
      const contents = await invoke('read_file', { path });
      setCurrentFile(path);
      setFileName(name);
      setCode(contents);
      setModified(false);
      setOutput(prev => prev + `\nOpened: ${name}\n`);
    } catch (e) {
      setOutput(prev => prev + `\nError opening file: ${e}\n`);
    }
  }, []);

  // Save current file
  const handleSave = useCallback(async () => {
    if (!currentFile) return;
    try {
      await invoke('save_file', { path: currentFile, contents: code });
      setModified(false);
      setOutput(prev => prev + `\nSaved: ${fileName}\n`);
    } catch (e) {
      setOutput(prev => prev + `\nError saving file: ${e}\n`);
    }
  }, [currentFile, code, fileName]);

  // Track modifications
  const handleCodeChange = useCallback((value) => {
    setCode(value);
    setModified(true);
  }, []);

  // Handle new file creation from toolbar
  useEffect(() => {
    const handleOpenFile = (e) => {
      const { path, name } = e.detail;
      handleFileSelect(path, name);
    };
    window.addEventListener('qbhd-open-file', handleOpenFile);
    return () => window.removeEventListener('qbhd-open-file', handleOpenFile);
  }, [handleFileSelect]);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        handleSave();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleSave]);

  return (
    <div className="app">
      <Toolbar
        currentFile={currentFile}
        setOutput={setOutput}
        onSave={handleSave}
        modified={modified}
      />
      <div className="main">
        <FileTree
          projectDir={projectDir}
          onFileSelect={handleFileSelect}
          onDirChange={setProjectDir}
        />
        <Editor
          code={code}
          setCode={handleCodeChange}
          onCursorChange={setCursorPos}
          onSave={handleSave}
        />
      </div>
      <Terminal output={output} setOutput={setOutput} />
      <StatusBar
        fileName={fileName}
        modified={modified}
        cursorPos={cursorPos}
        currentFile={currentFile}
      />
    </div>
  );
}

export default App;
