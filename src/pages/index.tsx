import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'next/router'
import Image from "next/image";


import backgroundImage from "../assets/background.svg"
import pirateMidiImage from "../assets/piratemidi.png"
import bridge6Image from "../assets/bridge6.svg"
import bridge4Image from "../assets/bridge4.svg"
import clickImage from "../assets/click.svg"
import uloopImage from "../assets/uloop.svg"




function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const router = useRouter();

  // listen for devices that have arrived
  useEffect(() => {
    const deviceConnected = async () => {
      await listen("device_connected", event => { router.push("/bridge") });
    }
    const deviceDisconnected = async () => {
      await listen("device_disconnected", event => { router.push("/") });
    }
    deviceConnected().catch(console.error);
    deviceDisconnected().catch(console.error);
  }, [router]);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="m-0 pt-[20vh] h-full flex flex-col justify-between container mx-auto text-center">
      <div>
        <h2 className="text-xl font-semibold">Connect one of the following devices to get started:</h2>
        <div className="flex justify-center">
          <span className="clickable-image">
            <a href="https://piratemidi.com/pages/bridge-6" target="_blank">
              <Image
                width={288}
                height={288}
                src={bridge6Image}
                className="!p-4 image-lift"
                alt="Bridge6 Image"
              />
            </a>
          </span>
          <span className="clickable-image">
            <a href="https://piratemidi.com/pages/bridge4" target="_blank">
              <Image
                width={288}
                height={288}
                src={bridge4Image}
                className="!p-4 image-lift"
                alt="Bridge4 Image"
              />
            </a>
          </span>
          {/* <span className="clickable-image">
            <a href="https://piratemidi.com/products/click-midi-interface-relay-switcher" target="_blank">
              <Image
                width={288}
                height={288}
                src={clickImage}
                className="!p-4 image-lift"
                alt="CLiCK Image"
              />
            </a>
          </span>
          <span className="clickable-image">
            <a href="https://piratemidi.com/products/%C2%B5loop-4-ch-bypass-and-midi-interface" target="_blank">
              <Image
                width={288}
                height={288}
                src={uloopImage}
                className="!p-4 image-lift"
                alt="uLOOP Image"
              />
            </a>
          </span> */}
        </div>

        <p>Click a device to learn more about Pirate MIDI's products.</p>
      </div>

      <span className="clickable-image">
        <a href="https://piratemidi.com/" target="_blank">
          <Image
            width={75}
            height={75}
            className="logo"
            src={pirateMidiImage}
            alt="Pirate MIDI Logo"
          />
        </a>
      </span>
    </div>
  );
}

export default App;
