import { Outlet, createRootRouteWithContext } from '@tanstack/react-router'
import { WindowWrapper } from '@/components/widgets/WindowWrapper'
import { RouterContext } from '@/App'

export const Route = createRootRouteWithContext<RouterContext>()({
  component: () => (
    <WindowWrapper>
      <Outlet />
    </WindowWrapper>
  ),
})
