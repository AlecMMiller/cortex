import { createFileRoute, Link } from '@tanstack/react-router'
import { useAvailableSchemas } from '@/commands/objects'
import { useQueryClient } from '@tanstack/react-query'
import { Plus } from 'lucide-react'
import { NavButton } from '@/components/ui/nav-button'
import { CreateSchemaDialog } from '@/components/dialogs/CreateSchema'

export const Route = createFileRoute('/objects')({
  component: RouteComponent,
})

function RouteComponent() {
  const { data: schemas, status } = useAvailableSchemas({})

  if (status !== 'success') {
    return <div>Loading</div>
  }

  const queryClient = useQueryClient()

  const schemaLinks = schemas.map((schema) => {
    return (
      <Link
        to={`/objects/${schema.uuid}`}
        className="p-2 text-blue bg-surface1 rounded-lg"
        key={schema.uuid}
      >
        {schema.name}
      </Link>
    )
  })

  return (
    <div className="m-4">
      {schemaLinks}
      <NavButton
        queryClient={queryClient}
        icon={Plus}
        tooltip="New Schema"
        DialogContent={CreateSchemaDialog}
      />
    </div>
  )
}
