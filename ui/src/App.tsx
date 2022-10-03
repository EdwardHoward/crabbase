import { BrowserRouter, Routes, Route } from 'react-router-dom'
import {
  QueryClient,
  QueryClientProvider,
} from 'react-query'
import Collections from './collections/Collections'
import Collection from './collections/Collection'
import './App.css'

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="App">
        <BrowserRouter basename="/_">
          <Routes>
            <Route path="/collections/:id" element={<Collection />} />
            <Route path="/collections" element={<Collections />} />
          </Routes>
        </BrowserRouter>
      </div>
    </QueryClientProvider>
  )
}

export default App
