// The first screen: a question, a text input and a button.
//
// A screen is a React component built from the kit's parts. Screen Compiler
// turns it into Rust when you build; nothing here runs on the device.
//   - `id` on a control that holds a value makes it a field your Rust reads.
//   - A callback prop (`onContinue`) becomes an action your Rust handles.
//   - A string prop (`nameError`) becomes a slot your Rust fills, and
//     `{props.x && ...}` a group it shows or hides.
import { Button, Column, Row, Screen, Spacer, Text, TextInput } from "nowayos:kit";

const GAP = 16;
const EDGE = 32;
const NAME_LENGTH = 40;

type WelcomeProps = {
  nameError: string;
  onContinue: () => void;
};

export function WelcomeScreen(props: WelcomeProps) {
  return (
    <Screen>
      <Column gap={GAP} padding={EDGE}>
        <Text role="title">Welcome</Text>
        <Text>What should we call you?</Text>
        <TextInput id="name" placeholder="Your name" maxLength={NAME_LENGTH} />
        {props.nameError && <Text>{props.nameError}</Text>}
        <Spacer />
        <Row gap={GAP}>
          <Spacer />
          <Button id="next" onPress={props.onContinue}>
            Continue
          </Button>
        </Row>
      </Column>
    </Screen>
  );
}
