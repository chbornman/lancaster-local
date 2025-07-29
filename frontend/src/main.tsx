import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import { HelmetProvider } from 'react-helmet-async'
import { DirectionProvider } from './contexts/DirectionContext'
import App from './App'
import './i18n'
import './index.css'

const rootElement = document.getElementById('root')

if (!rootElement) {
  throw new Error('Failed to find the root element')
}

ReactDOM.createRoot(rootElement).render(
  <React.StrictMode>
    <HelmetProvider>
      <BrowserRouter>
        <DirectionProvider>
          <App />
        </DirectionProvider>
      </BrowserRouter>
    </HelmetProvider>
  </React.StrictMode>,
)