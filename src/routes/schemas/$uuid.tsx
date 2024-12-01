import {
  ensureSchema,
  renameSchema,
  useSchemaSuspense,
} from '@/commands/objects'
import { useQueryClient } from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import { useRef, useState } from 'react'

export const Route = createFileRoute('/schemas/$uuid')({
  loader: ({ context, params }) =>
    ensureSchema(context.queryClient, {}, params.uuid),
  component: RouteComponent,
})

function onNameChange(client: QueryClient, uuid: string, name: string): void {
  renameSchema(client, uuid, name)
}

interface ContentEditableProps {
  value: string
  onChange: (val: string) => void
  className?: string
}

const ContentEditableWithRef = (props: ContentEditableProps) => {
  const defaultValue = useRef(props.value)

  const handleInput = (e) => {
    console.log(e)
    if (props.onChange) {
      props.onChange(e.target.textContent)
    }
  }

  return (
    <span
      className={props.className}
      contentEditable="plaintext-only"
      onInput={handleInput}
      dangerouslySetInnerHTML={{ __html: defaultValue.current }}
    />
  )
}

function RouteComponent() {
  const { uuid } = Route.useParams()

  const { data: schema } = useSchemaSuspense({}, uuid)

  const client = useQueryClient()

  return (
    <div>
      <ContentEditableWithRef
        className="text-text text-2xl"
        value={schema.name}
        onChange={(val: string) => {
          onNameChange(client, uuid, val)
        }}
      />
    </div>
  )
}
