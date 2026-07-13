export default function StatusBar({ fileName, modified, cursorPos, currentFile }) {
  return (
    <div className="status-bar">
      <div className="status-left">
        {currentFile ? (
          <>
            <span className="status-item">
              {modified ? '●' : '○'} {fileName || 'Untitled'}
            </span>
            <span className="status-item">BASIC</span>
          </>
        ) : (
          <span className="status-item">No file open</span>
        )}
      </div>
      <div className="status-right">
        <span className="status-item">
          Ln {cursorPos.line}, Col {cursorPos.column}
        </span>
        <span className="status-item">UTF-8</span>
        <span className="status-item">QBHD IDE</span>
      </div>
    </div>
  );
}
