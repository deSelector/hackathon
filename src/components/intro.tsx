import "./styles.scss";
import classnames from "classnames";
import React from "react";
import { useDataContext, IntroType } from "../context";

export function Intro() {
  const { showIntro, setShowIntro } = useDataContext();

  return (
    <div className={classnames("instructions-panel", { showIntro })}>
      <div>
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed finibus risus ut feugiat gravida. Integer vitae sem
        iaculis, congue odio vel, iaculis leo. Integer condimentum tristique est eu semper. Duis quis finibus nibh. Cras
        ut lobortis felis. Donec faucibus felis eu purus interdum auctor. Cras dignissim justo et lorem tempus, a rutrum
        nisi dictum. Aenean egestas, erat et congue fermentum, est libero iaculis tellus, vitae iaculis leo sapien sed
        lorem. Curabitur ut leo nibh. Phasellus ac lectus vel est blandit ornare. Mauris eget lacus vestibulum, faucibus
        augue id, faucibus risus. Phasellus et euismod libero, ac fringilla dui. Nam feugiat, nulla id mollis fermentum,
        lectus purus dapibus mi, sit amet porttitor libero erat a sem. Morbi ornare id magna id ornare. Praesent
        aliquet, ligula at viverra porttitor, ipsum lorem blandit metus, vel fermentum sapien metus quis velit. Morbi
        risus est, molestie eu elit vel, consequat posuere quam. Nulla congue vel massa nec congue. Vivamus elementum
        ante augue, vel pharetra massa condimentum eu. Phasellus sagittis rhoncus sem, sed vulputate metus euismod in.
        Donec finibus, sapien eget congue accumsan, quam purus ultrices orci, in varius elit ligula in leo. In lectus
        magna, lobortis vel rutrum ut, eleifend vel purus. Pellentesque metus nisi, pulvinar in ipsum non, iaculis
        tincidunt ex. Pellentesque sed suscipit dolor. Duis vel aliquam mi. Donec nec libero in orci interdum commodo.
        In a laoreet ipsum. Etiam vel lorem nisl. Pellentesque massa mauris, tempus at consectetur non, fringilla eget
        massa. Vivamus id felis semper justo auctor mollis ac in tellus. Sed viverra posuere orci. Fusce ac suscipit
        nibh. Nam volutpat feugiat enim sit amet hendrerit. Cras ornare nunc aliquam quam suscipit porta. Phasellus
        vulputate velit at urna interdum, vel maximus odio commodo. Morbi libero diam, venenatis quis ipsum non, rhoncus
        mattis nisl. Interdum et malesuada fames ac ante ipsum primis in faucibus. Mauris tincidunt, tortor et bibendum
        dapibus, ex quam imperdiet magna, non volutpat erat urna id ante. Suspendisse vel porta mauris, et consequat
        massa. Phasellus sed nulla rhoncus tellus ultrices tincidunt sit amet in leo. Donec leo tortor, commodo vitae
        volutpat vitae, egestas non turpis. Curabitur placerat sapien nibh, ac vestibulum mi sagittis at. Etiam lobortis
        eget orci ac accumsan. Aenean ut ultricies nulla. In hac habitasse platea dictumst. Aliquam ac mi finibus,
        elementum nibh quis, scelerisque nunc. Aliquam rhoncus lorem orci, id dignissim odio finibus ut. Nullam tempus
        condimentum leo et vehicula. Praesent scelerisque orci gravida purus rutrum aliquam.
      </div>
      <button onClick={() => setShowIntro(IntroType.none)}>{"got it"}</button>
    </div>
  );
}
