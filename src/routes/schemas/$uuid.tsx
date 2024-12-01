import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/schemas/$uuid')({
  component: RouteComponent,
})

function RouteComponent() {
  const { uuid } = Route.useParams()

  return <div>{uuid}</div>
}
