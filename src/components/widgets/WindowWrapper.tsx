import { getCurrentWindow } from '@tauri-apps/api/window'
import { useState, useEffect } from 'react'
import { Sidebar } from './Sidebar'
import { Topbar } from './Topbar'

interface WindowWrapperProps {
  children: React.ReactNode
}

export function WindowWrapper(props: WindowWrapperProps): JSX.Element {
  const appWindow = getCurrentWindow()

  const [isFullScreen, setIsFullScreen] = useState(false)

  const checkIsFullScreen = async (): Promise<void> => {
    const state = await appWindow.isMaximized()
    setIsFullScreen(state)
  }

  useEffect(() => {
    checkIsFullScreen()
  }, [appWindow])

  const rounded = isFullScreen ? '' : 'rounded-md'

  return (
    <div
      className={`bg-crust text-3xl w-screen h-screen flex flex-col ${rounded}`}
    >
      <Topbar
        handleClose={() => appWindow.close()}
        handleMaximize={() => {
          setIsFullScreen(!isFullScreen)
          appWindow.toggleMaximize()
          checkIsFullScreen()
        }}
        handleMinimize={() => {
          appWindow.minimize()
        }}
      />
      <div className="flex flex-row justify-between flex-1 min-h-0">
        <Sidebar />
        <div className="bg-base w-full flex flex-row rounded-tl-md min-h-0">
          {props.children}
        </div>
      </div>
    </div>
  )
}
