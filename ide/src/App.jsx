import { useState } from 'react';
import Editor from './components/Editor';
import FileTree from './components/FileTree';
import Terminal from './components/Terminal';
import Toolbar from './components/Toolbar';
import './App.css';

function App() {
  const [currentFile, setCurrentFile] = useState(null);
  const [code, setCode] = useState('');
  const [output, setOutput] = useState('');

  return (
    <div className="app">
      <Toolbar currentFile={currentFile} code={code} setOutput={setOutput} />
      <div className="main">
        <FileTree onFileSelect={setCurrentFile} />
        <Editor code={code} setCode={setCode} />
      </div>
      <Terminal output={output} />
    </div>
  );
}

export default App;
