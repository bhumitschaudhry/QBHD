import MonacoEditor from '@monaco-editor/react';

export default function Editor({ code, setCode }) {
  return (
    <div className="editor">
      <MonacoEditor
        height="100%"
        language="basic"
        theme="vs-dark"
        value={code}
        onChange={setCode}
        options={{
          minimap: { enabled: true },
          fontSize: 14,
          lineNumbers: 'on',
          roundedSelection: true,
          scrollBeyondLastLine: false,
          automaticLayout: true,
        }}
      />
    </div>
  );
}
