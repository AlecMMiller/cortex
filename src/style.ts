export const BASE_TEXT = 'text-text-normal font-prose'

export const NORMAL_TEXT = `${BASE_TEXT} text-base lg:text-lg font-semibold`

export const EDITOR_THEME = {
  paragraph: `${NORMAL_TEXT} py-4 leading-loose'`,
  text: {
    bold: 'font-extrabold text-text-bold',
    strikethrough: 'line-through',
    code: 'bg-background-code p-1 rounded-md'
  },
  quote: `${NORMAL_TEXT} border-l-4 border-quote pl-4`,
  hr: 'border-0 bg-separator h-px',
  heading: {
    h1: 'text-text-normal font-prose font-semibold text-lg lg:text-5xl py-2 my-2 border-b border-separator',
    h2: 'text-text-normal font-prose font-semibold text-lg lg:text-4xl py-2 my-2 border-b border-separator',
    h3: 'text-text-normal font-prose font-semibold text-lg lg:text-3xl py-4',
    h4: 'text-text-normal font-prose font-semibold text-lg lg:text-2xl py-4',
    h5: 'text-text-normal font-prose font-semibold text-lg lg:text-xl py-4',
    h6: 'text-text-normal font-prose font-semibold text-lg lg:text-lg py-4',
  },
  code: 'text-lg lg:text-xl text-text-normal bg-background-code py-0 border-1 block rounded-md p-4',
  list: {
    nested: {
      listitem: ''
    },
    listitem: `${NORMAL_TEXT} list-disc ml-6`,
    codeHighlight: {
      builtin: 'text-text-bold',
    }
  },
  link: 'text-text-bold'
}