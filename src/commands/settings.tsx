import { commands } from '@/bindings'
import { buildQueryMethods } from './common'

export const { useType: useGetSettingOrSet } = buildQueryMethods(
  commands.getSettingOrSet,
  (key: string, ..._rest) => ['setting', key],
)

export function makeSettingKey(settingKey: string) {
  return ['setting', settingKey]
}

export const { useType: useSetting } = buildQueryMethods(
  commands.getSetting,
  makeSettingKey,
)

export async function updateSetting(key: string, value: string) {
  await commands.updateSetting(key, value)
}
