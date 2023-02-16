import type { AppProps } from "next/app";
import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'next/router'
import { useEffect, useState, createContext, useContext } from 'react';


import type { ConnectedDevice } from '../../src-tauri/bindings/ConnectedDevice';

import "../style.css";

// This default export is required in a new `pages/_app.js` file.
export default function Ahoy({ Component, pageProps }: AppProps) {
  const router = useRouter()
  const [devices, setDevices] = useState<ConnectedDevice[]>([])
  const [selected, setSelected] = useState<ConnectedDevice>(undefined)
  const [isInstalling, setIsInstalling] = useState<boolean>(false)


  // listen for devices being plugged and unplugged
  useEffect(() => {
    // flag to keep track of listeners
    let shouldListen = true;

    // define arrival listener
    async function deviceArrivals() {
      const listener = await listen<ConnectedDevice>('device_connected', arrived => {
        console.log(arrived)
        shouldListen ? setDevices(current => [...current, arrived.payload]) : () => { }
      })
      return !shouldListen ? listener() : () => { }
    }

    // define departature listener
    async function deviceDepartures() {
      const listener = await listen<ConnectedDevice>('device_disconnected', leaving => {
        console.log(leaving)
        shouldListen ? setDevices(current => current.filter(device => device.serial_number !== leaving.payload.serial_number)) : () => { }
      })
      return !shouldListen ? listener() : () => { }
    }

    // route depending on state
    router.push(devices.length > 0 ? '/devices' : '/')

    // start our listeners
    deviceArrivals().catch(console.error)
    deviceDepartures().catch(console.error)

    // destructor
    return () => {
      shouldListen = false
    }
  }, [devices])

  // return main component
  return (
    <Component {...pageProps} devices={devices} />
  )
}
