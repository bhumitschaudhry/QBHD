export default function Terminal({ output }) {
  return (
    <div className="terminal">
      <div className="terminal-header">Terminal</div>
      <pre className="terminal-content">{output}</pre>
    </div>
  );
}
