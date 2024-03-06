import React from "react"
// @ts-ignore
import Editor, { EventInfo } from "ckeditor5-custom-build"

import { CKEditor } from "@ckeditor/ckeditor5-react"

const editorConfiguration = {
  toolbar: [
    "heading",
    "|",
    "bold",
    "italic",
    "link",
    "bulletedList",
    "numberedList",
    "|",
    "outdent",
    "indent",
    "|",
    "imageUpload",
    "blockQuote",
    "insertTable",
    "mediaEmbed",
    "undo",
    "redo",
  ],
}

function CustomEditor(props: {
  initialData?: string
  onChange?: (event: EventInfo<string, unknown>, editor: Editor) => void
}) {
  return (
    <CKEditor
      editor={Editor}
      config={editorConfiguration}
      data={props.initialData}
      onChange={props.onChange}
    />
  )
}

export default CustomEditor
