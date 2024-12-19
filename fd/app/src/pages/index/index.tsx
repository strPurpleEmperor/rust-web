import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import eruda from 'eruda';
const rootEl = document.getElementById('root');
eruda.init();
if (rootEl) {
  const root = ReactDOM.createRoot(rootEl);
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
}
