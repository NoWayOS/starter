// The second screen: it greets whoever the first screen asked about.
//
// `props.name` is a slot: the application sets its text (src/lib.rs), and
// the screen shows it inside the words around it.
import { Button, Column, Row, Screen, Spacer, Text } from "nowayos:kit";

const GAP = 16;
const EDGE = 32;

type HelloProps = {
  name: string;
  onStartAgain: () => void;
};

export function HelloScreen(props: HelloProps) {
  return (
    <Screen>
      <Column gap={GAP} padding={EDGE}>
        <Text id="greeting" role="title">Hello, {props.name}</Text>
        <Text lines={2}>
          {"You wrote this screen in React. It is running as native Rust on seL4, with no browser anywhere."}
        </Text>
        <Spacer />
        <Row gap={GAP}>
          <Button id="start-again" onPress={props.onStartAgain}>
            Start again
          </Button>
        </Row>
      </Column>
    </Screen>
  );
}
