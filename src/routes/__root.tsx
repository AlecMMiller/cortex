import { Outlet, createRootRoute } from '@tanstack/react-router'
import { WindowWrapper } from '@/components/widgets/WindowWrapper'

export const Route = createRootRoute({
  component: () => (
    <>
      <WindowWrapper>
        <Outlet />
      </WindowWrapper>
    </>
  ),
})
