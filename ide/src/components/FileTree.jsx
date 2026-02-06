import { useState, useEffect } from 'react';
import { readDir } from '@tauri-apps/api/fs';

export default function FileTree({ onFileSelect }) {
  const [files, setFiles] = useState([]);

  useEffect(() => {
    loadFiles();
  }, []);

  const loadFiles = async () => {
    try {
      const entries = await readDir('.', { recursive: false });
      setFiles(entries.filter(e => e.name.endsWith('.bas')));
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="file-tree">
      <h3>Files</h3>
      {files.map(file => (
        <div key={file.path} onClick={() => onFileSelect(file.path)} className="file-item">
          📄 {file.name}
        </div>
      ))}
    </div>
  );
}
