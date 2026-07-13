import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export default function FileTree({ projectDir, onFileSelect, onDirChange }) {
  const [entries, setEntries] = useState([]);
  const [expandedDirs, setExpandedDirs] = useState(new Set());
  const [dirContents, setDirContents] = useState({});

  const loadDirectory = useCallback(async (dirPath) => {
    try {
      const result = await invoke('list_directory', { path: dirPath });
      return result;
    } catch (e) {
      console.error('Failed to list directory:', e);
      return [];
    }
  }, []);

  // Load root directory on mount and when projectDir changes
  useEffect(() => {
    loadDirectory(projectDir).then(setEntries);
  }, [projectDir, loadDirectory]);

  const handleFileClick = async (entry) => {
    if (entry.is_dir) {
      // Toggle directory expansion
      const newExpanded = new Set(expandedDirs);
      if (newExpanded.has(entry.path)) {
        newExpanded.delete(entry.path);
      } else {
        newExpanded.add(entry.path);
        // Load directory contents if not already loaded
        if (!dirContents[entry.path]) {
          const contents = await loadDirectory(entry.path);
          setDirContents(prev => ({ ...prev, [entry.path]: contents }));
        }
      }
      setExpandedDirs(newExpanded);
    } else {
      onFileSelect(entry.path, entry.name);
    }
  };

  const isBASICFile = (name) => {
    const lower = name.toLowerCase();
    return lower.endsWith('.bas') || lower.endsWith('.bi') || lower.endsWith('.bm');
  };

  const getFileIcon = (entry) => {
    if (entry.is_dir) {
      return expandedDirs.has(entry.path) ? '📂' : '📁';
    }
    if (isBASICFile(entry.name)) return '📄';
    if (entry.name.endsWith('.txt') || entry.name.endsWith('.md')) return '📝';
    if (entry.name.endsWith('.json')) return '📋';
    if (entry.name.endsWith('.js') || entry.name.endsWith('.jsx')) return '⚡';
    if (entry.name.endsWith('.rs')) return '🦀';
    return '📄';
  };

  const renderEntry = (entry, depth = 0) => {
    const isExpanded = expandedDirs.has(entry.path);
    const children = dirContents[entry.path] || [];

    return (
      <div key={entry.path}>
        <div
          className={`file-item ${entry.is_dir ? 'dir-item' : 'file-leaf'}`}
          style={{ paddingLeft: `${10 + depth * 16}px` }}
          onClick={() => handleFileClick(entry)}
        >
          <span className="file-icon">{getFileIcon(entry)}</span>
          <span className="file-name">{entry.name}</span>
        </div>
        {entry.is_dir && isExpanded && (
          <div className="dir-children">
            {children.map(child => renderEntry(child, depth + 1))}
          </div>
        )}
      </div>
    );
  };

  const handleRefresh = async () => {
    const result = await loadDirectory(projectDir);
    setEntries(result);
    // Refresh expanded dirs too
    for (const dirPath of expandedDirs) {
      const contents = await loadDirectory(dirPath);
      setDirContents(prev => ({ ...prev, [dirPath]: contents }));
    }
  };

  const handleOpenFolder = async () => {
    // Use Tauri dialog to select folder
    try {
      const { open } = await import('@tauri-apps/api/dialog');
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Open Project Folder',
      });
      if (selected) {
        onDirChange(selected);
      }
    } catch (e) {
      console.error('Failed to open folder dialog:', e);
    }
  };

  return (
    <div className="file-tree">
      <div className="file-tree-header">
        <span className="file-tree-title">Explorer</span>
        <div className="file-tree-actions">
          <button className="icon-btn" onClick={handleRefresh} title="Refresh">↻</button>
          <button className="icon-btn" onClick={handleOpenFolder} title="Open Folder">📂</button>
        </div>
      </div>
      <div className="file-tree-content">
        {entries.length === 0 ? (
          <div className="file-tree-empty">
            No files found.
            <br />
            <button className="link-btn" onClick={handleOpenFolder}>
              Open a folder
            </button>
          </div>
        ) : (
          entries.map(entry => renderEntry(entry))
        )}
      </div>
    </div>
  );
}
