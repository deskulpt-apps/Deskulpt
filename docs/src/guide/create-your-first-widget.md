# Create Your First Widget

Each Deskulpt widget is a folder placed in the designated widgets directory on your system. To find that directory, open the Deskulpt manager interface, go to the "Widgets" tab, and click the "Open Widgets Directory" button in the bottom-left corner.

## Manifest

Not all folders under the widgets directory are considered Deskulpt widgets. A valid Deskulpt widget must contain a `deskulpt.widget.json` manifest file at the root of the folder. This manifest file provides essential metadata about the widget. Let's create a simple manifest file for our first "Counter" widget. Create a new folder named `counter` in the widgets directory, and inside that folder, create a file named `deskulpt.widget.json` with the following content:

```json
{
  "name": "Counter",
  "entry": "index.jsx"
}
```

See [Manifest Options](#manifest-options) for more information you can include in the manifest file.

## Widget Component

Now, let's create the main widget component file `index.jsx` in the same folder. Note how the manifest's `entry` field points to this file. A Deskulpt widget is essentially a React component. Here is a simple implementation of a counter widget:

```jsx
import { useState } from "@deskulpt-test/react";
import { Button, Flex, Heading } from "@deskulpt-test/ui";

export default function SimpleCounter() {
  const [count, setCount] = useState(0);

  return (
    <Flex height="100%" width="100%" align="center" justify="center" gap="3">
      <Button variant="surface" onClick={() => setCount(count - 1)}>
        Decrement
      </Button>
      <Heading size="6">{count}</Heading>
      <Button variant="surface" onClick={() => setCount(count + 1)}>
        Increment
      </Button>
    </Flex>
  );
}
```

Note how this is almost identical to a standard React component in web development. The only difference is that instead of importing from `react`, we import from `@deskulpt-test/react`. All React hooks and APIs are identically available through this package. The component must be exported as the default export of the entry file specified in the manifest, so Deskulpt can pick it up properly.

Also, `@deskulpt-test/ui` is a partial re-export of [Radix Themes](https://www.radix-ui.com/), which provides some pre-built UI components that align with Deskulpt's design system.

## Rendering the Widget

To see your widget in action, open the Deskulpt manager interface, and under the "Widgets" tab, click the "Refresh All" button in the bottom-left corner. The folder name `counter` should now appear in the list of available widgets and it should be rendered on your desktop already. Click the `counter` entry in the list to adjust its settings if needed, or you can drag and resize it directly on your desktop.

Note that every time you make changes to the widget's code, you will need to click the "Refresh" button for that specific widget to see the updates reflected on your desktop. You can also click "Refresh All" if you prefer, but note that it will refresh all widgets, not just the one you modified.

## Manifest Options

- `name` (string, required): The display name of the widget.
- `entry` (string, required): The relative path to the main widget component file.
- `version` (string, optional): The version of the widget.
- `authors` (list, optional): The authors of the widget. Each author can either be described as a string (author name), or an object with `name` (required), `email` (optional), and `homepage` (optional) fields.
- `license` (string, optional): The license under which the widget is distributed.
- `description` (string, optional): A brief description of the widget.
- `homepage` (string, optional): The URL of the widget's homepage.
- `ignore` (boolean, optional): If set to true, Deskulpt will completely ignore this folder and not treat it as a Deskulpt widget, despite the presence of a manifest file.
