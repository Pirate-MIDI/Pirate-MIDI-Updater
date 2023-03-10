import type { AppProps } from "next/app";
import { listen, emit } from '@tauri-apps/api/event'
import { useRouter } from 'next/router'
import { useEffect, useState, createContext, useContext } from 'react';

import { ConnectedDevice } from '../../src-tauri/bindings/ConnectedDevice';

import "../style.css";
import { InstallerState } from "../../src-tauri/bindings/InstallerState";

// This default export is required in a new `pages/_app.js` file.
export default function Ahoy({ Component, pageProps }: AppProps) {
  const router = useRouter()
  const [devices, setDevices] = useState<ConnectedDevice[]>([])
  const [installerState, setInstallerState] = useState<InstallerState>({ type: "Init" })

  // listen for devices being plugged and unplugged
  useEffect(() => {
    // event listeners
    const deviceListener = listen<ConnectedDevice[]>('devices_update', event => setDevices(event.payload))
    const installerStateListener = listen<InstallerState>('installer_state', event => setInstallerState(event.payload))

    // tell the backend we're ready for events
    emit('ready')

    // destructor
    return () => {
      deviceListener.then(f => f())
      installerStateListener.then(f => f())
    }
  }, [])

  // force routing should state change
  useEffect(() => {
    console.log("routing: ", installerState.type)
    switch (installerState.type) {
      case "Init":
        void router.replace(devices.length > 0 ? '/devices' : '/')
        break
      case "Bootloader":
        console.log(installerState.device)
        void router.replace({
          pathname: '/install',
          query: { device_type: installerState.device.device_type }
        }, '/releases')
        break
      case "PostInstall":
        void router.replace('/postinstall')
    }
  }, [devices, installerState])

  // return main component
  return (
    <Component {...pageProps} devices={devices} className="overflow-hidden" />
  )
}
