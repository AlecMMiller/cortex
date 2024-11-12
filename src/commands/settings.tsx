import { invoke } from '@tauri-apps/api/core'
import { commands } from '@/bindings'
import { newBuildQueryMethods } from './common'
type Setting = {
  key: string
  value: string
}

export const { useType: useGetSettingOrSet } = newBuildQueryMethods(
  commands.getSettingOrSet,
  (key: string, ..._rest) => ['setting', key],
)

export function makeSettingKey(settingKey: string) {
  return ['setting', settingKey]
}

export const { useType: useSetting } = newBuildQueryMethods(
  commands.getSetting,
  makeSettingKey,
)

export async function updateSetting(setting: Setting) {
  await invoke('update_setting', setting)
}
