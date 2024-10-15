import { TooltipButton } from '@/components/ui/button-tooltip'
import { Link, LucideIcon, TableOfContents, Tag } from 'lucide-react'
import { TableOfContentsNavigator, TocContents } from './TableOfContents'
import { useState } from 'react'

interface TabProps<TabEnum> {
  currentTab: TabEnum
  icon: LucideIcon
  tooltip: string
  id: TabEnum
  setCurrentTab: (tab: TabEnum) => void
}

function Tab<TabEnum>(props: TabProps<TabEnum>): JSX.Element {
  const isSelected = props.currentTab === props.id
  const Icon = props.icon

  let icon = (
    <TooltipButton
      onClick={() => props.setCurrentTab(props.id)}
      tooltip={props.tooltip}
      variant="ghost"
      size="fit"
    >
      <Icon size={14} className="text-subtext0" />
    </TooltipButton>
  )

  const baseClass = 'p-2 py-0'

  if (isSelected) {
    return (
      <div className={`${baseClass} relative bg-base rounded-t-md`}>
        <div className="absolute -left-1.5 bottom-0 w-1.5 h-1 bg-base">
          <div className="bg-mantle rounded-br-lg w-1.5 h-1" />
        </div>
        {icon}
        <div className="absolute -right-1.5 bottom-0 w-1.5 h-1 bg-base">
          <div className="bg-mantle rounded-bl-lg w-1.5 h-1" />
        </div>
      </div>
    )
  } else {
    return <div className={`${baseClass}`}>{icon}</div>
  }
}

enum NavigatorTab {
  ToC = 'toc',
  Tags = 'tags',
  Links = 'links',
}

interface PageNavigatorProps {
  toc: TocContents
}

export default function PageNavigator(props: PageNavigatorProps): JSX.Element {
  const [currentTab, setCurrentTab] = useState(NavigatorTab.ToC)
  console.log(currentTab)
  const tabs = [
    <Tab
      setCurrentTab={setCurrentTab}
      id={NavigatorTab.ToC}
      currentTab={currentTab}
      tooltip="Contents"
      key={NavigatorTab.ToC}
      icon={TableOfContents}
    />,
    <Tab
      setCurrentTab={setCurrentTab}
      id={NavigatorTab.Tags}
      currentTab={currentTab}
      tooltip="Tags"
      key={NavigatorTab.Tags}
      icon={Tag}
    />,
    <Tab
      setCurrentTab={setCurrentTab}
      id={NavigatorTab.Links}
      currentTab={currentTab}
      tooltip="Links"
      key={NavigatorTab.Links}
      icon={Link}
    />,
  ]

  let body = <></>

  if (currentTab === NavigatorTab.ToC) {
    body = <TableOfContentsNavigator toc={props.toc} />
  }

  return (
    <div className="flex flex-col bg-mantle">
      <div className="flex flex-row mt-1 px-1.5">{tabs}</div>
      <div className="grow w-56 bg-base">{body}</div>
    </div>
  )
}
