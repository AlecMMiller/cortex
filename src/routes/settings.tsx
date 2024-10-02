import { makeSettingKey, updateSetting, useSetting } from '@/commands/settings'
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectItem,
  SelectContent,
} from '@/components/ui/select'
import { useQueryClient } from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import { useEffect, useState } from 'react'

interface OptionProps {
  value: string
  display: string
}

interface OptionSelectProps {
  settingKey: string
  options: OptionProps[]
}

function OptionSelect(props: OptionSelectProps) {
  const [selectedOption, setSelectedOption] = useState('en-US')
  const { options } = props
  const { data } = useSetting({ key: props.settingKey }, {})
  const queryClient = useQueryClient()

  useEffect(() => {
    if (data !== undefined) {
      const current = options.find((option) => option.value === data.value)
      if (current !== undefined) setSelectedOption(current.value)
    }
  }, [data])

  const items = props.options.map((option) => {
    return (
      <SelectItem key={option.value} value={option.value}>
        {option.display}
      </SelectItem>
    )
  })

  const doChange = async (choice: string) => {
    setSelectedOption(choice)
    await updateSetting({ key: props.settingKey, value: choice })
    const queryKey = makeSettingKey(props.settingKey)
    queryClient.invalidateQueries({ queryKey })
  }

  return (
    <Select
      value={selectedOption}
      onValueChange={(value) => {
        doChange(value)
      }}
    >
      <SelectTrigger>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>{items}</SelectContent>
    </Select>
  )
}

export const Route = createFileRoute('/settings')({
  component: () => (
    <div>
      <OptionSelect
        settingKey="locale"
        options={[
          { value: 'en-US', display: 'English' },
          { value: 'ja', display: '日本語' },
        ]}
      />
    </div>
  ),
})
