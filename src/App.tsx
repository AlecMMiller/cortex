import { WindowWrapper } from './components/widgets/WindowWrapper'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import { TestPage } from './routes/Test'

const router = createBrowserRouter([
  {
    path: '/',
    element: <TestPage />,
  },
])

function App(): JSX.Element {
  return (
    <WindowWrapper>
      <RouterProvider router={router} />
    </WindowWrapper>
  )
}

export default App
