import { useRef, useEffect } from 'react';

export default function Terminal({ output, setOutput }) {
  const contentRef = useRef(null);

  // Auto-scroll to bottom
  useEffect(() => {
    if (contentRef.current) {
      contentRef.current.scrollTop = contentRef.current.scrollHeight;
    }
  }, [output]);

  const handleClear = () => {
    setOutput('');
  };

  return (
    <div className="terminal">
      <div className="terminal-header">
        <span className="terminal-title">Output</span>
        <div className="terminal-actions">
          <button className="icon-btn" onClick={handleClear} title="Clear output">🗑</button>
        </div>
      </div>
      <pre ref={contentRef} className="terminal-content">{output}</pre>
    </div>
  );
}
