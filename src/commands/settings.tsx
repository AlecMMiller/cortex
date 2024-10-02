import { invoke } from '@tauri-apps/api/core'
import { buildQueryMethods } from './common'
type Setting = {
  key: string
  value: string
}

export const { useType: useGetSettingOrSet } = buildQueryMethods<
  Setting,
  Setting
>({
  command: 'get_setting_or_set',
  makeKey: (data: Setting) => {
    return ['setting', data.key, data.value]
  },
})

type GetSetting = {
  key: string
}

export function makeSettingKey(settingKey: string) {
  return ['setting', settingKey]
}

export const { useType: useSetting } = buildQueryMethods<GetSetting, Setting>({
  command: 'get_setting',
  makeKey: (data: GetSetting) => {
    return makeSettingKey(data.key)
  },
})

export async function updateSetting(setting: Setting) {
  await invoke('update_setting', setting)
}
