import { BrowserRouter, Routes, Route } from 'react-router-dom'
import {
  QueryClient,
  QueryClientProvider,
} from 'react-query'
import Collections from './collections/Collections'
import Collection from './collections/Collection'
import './App.css'
import Layout from './Layout'
import EditCollection from './collections/EditCollection'
import CreateCollection from './collections/CreateCollection'

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="App">
        <BrowserRouter basename="/_">
          <Routes>
            <Route path="/" element={<Layout />}>
              <Route path="/collections" element={<Collections />} />
              <Route path="/collections/create" element={<CreateCollection />} />
              <Route path="/collections/:id" element={<Collection />} />
              <Route path="/collections/:id/edit" element={<EditCollection />} />
            </Route>
          </Routes>
        </BrowserRouter>
      </div>
    </QueryClientProvider>
  )
}

export default App
