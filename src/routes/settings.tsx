import { makeSettingKey, updateSetting, useSetting } from '@/commands/settings'
import { Select, SelectOption } from '@/components/ui/select'
import { useQueryClient } from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
interface OptionSelectProps {
  readonly settingKey: string
  readonly options: SelectOption[]
}

function OptionSelect(props: OptionSelectProps) {
  const { options } = props
  const { data } = useSetting({}, props.settingKey)
  const queryClient = useQueryClient()

  const onChange = async (choice: string) => {
    await updateSetting(props.settingKey, choice)
    const queryKey = makeSettingKey(props.settingKey)
    queryClient.invalidateQueries({ queryKey })
  }

  return (
    <Select value={data?.value} options={options} onValueChange={onChange} />
  )
}

export const Route = createFileRoute('/settings')({
  component: () => (
    <div>
      <OptionSelect
        settingKey="locale"
        options={[
          { value: 'en-US', content: 'English' },
          { value: 'ja', content: '日本語' },
        ]}
      />
    </div>
  ),
})
