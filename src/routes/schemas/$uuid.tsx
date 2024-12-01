import { ensureSchema, useSchemaSuspense } from '@/commands/objects'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/schemas/$uuid')({
  loader: ({ context, params }) =>
    ensureSchema(context.queryClient, {}, params.uuid),
  component: RouteComponent,
})

function RouteComponent() {
  const { uuid } = Route.useParams()

  const { data: schema } = useSchemaSuspense({}, uuid)

  return (
    <div>
      <h1 className="text-4xl text-text">{schema.name}</h1>
    </div>
  )
}
