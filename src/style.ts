export const BASE_TEXT = 'text-text font-prose'

export const NORMAL_TEXT = `${BASE_TEXT} lg:text-lg font-semibold`

export const EDITOR_THEME = {
  paragraph: `${NORMAL_TEXT} py-4 leading-loose'`,
  text: {
    bold: 'font-extrabold text-text',
    strikethrough: 'line-through',
    code: 'bg-crust rounded-md text-text p-1'
  },
  quote: 'text-subtext0 border-l-4 border-overlay1 pl-4 lg:text-lg',
  hr: 'border-0 bg-separator h-px',
  heading: {
    h1: 'text-text font-prose font-semibold text-lg lg:text-5xl py-2 my-2 border-b border-overlay1',
    h2: 'text-text font-prose font-semibold text-lg lg:text-4xl py-2 my-2 border-b border-overlay1',
    h3: 'text-subtext0 font-prose font-semibold text-lg lg:text-3xl py-4',
    h4: 'text-subtext0 font-prose font-semibold text-lg lg:text-2xl py-4',
    h5: 'text-subtext1 font-prose font-semibold text-lg lg:text-xl py-4',
    h6: 'text-subtext1 font-prose font-semibold text-lg lg:text-lg py-4'
  },
  code: 'text-lg lg:text-xl text-text bg-crust border-1 block rounded-md p-2',
  list: {
    nested: {
      listitem: ''
    },
    listitem: `${NORMAL_TEXT} list-disc ml-6`,
    codeHighlight: {
      builtin: 'text-text'
    }
  },
  link: 'text-text'
}
