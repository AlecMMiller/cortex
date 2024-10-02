import { invoke } from '@tauri-apps/api/core'

interface Setting {
  setting_key: string
  value: string
}

export async function getOrSetSetting(
  key: string,
  value: string,
): Promise<string> {
  const result = (await invoke('get_setting_or_set', {
    key,
    value,
  })) as Setting
  return result.value
}
