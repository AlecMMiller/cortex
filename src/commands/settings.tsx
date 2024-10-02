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
