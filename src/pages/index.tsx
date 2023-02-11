import Image from 'next/image';
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'next/router'
import { useMachine } from '@xstate/react';
import { createMachine } from 'xstate';
import { useEffect, useState } from 'react';

import uloopImage from '../assets/uloop.svg'
import clickImage from '../assets/click.svg'
import bridge4Image from '../assets/bridge4.svg'
import bridge6Image from '../assets/bridge6.svg'
import pirateMidiImage from '../assets/piratemidi.png'

import { ConnectedDevice } from '../../src-tauri/bindings/ConnectedDevice';

function App() {
  const router = useRouter()
  const [devices, setDevices] = useState<ConnectedDevice[]>([])

  // use a global state machine for the installation process
  // the devices will disconnect and reconnect as they enter/exit bootloader mode
  // this could throw the app into a weird state from a user experience
  // thus why we use a state machine - to make this more seamless
  const installerMachine = createMachine({
    id: 'installer',
    initial: 'initial',
    states: {
      initial: {
        on: {
          CONNECTED: 'connected'
        }
      },
      connected: {
        invoke: {
          src: 'fetchReleases',
          onDone: 'releaseSelection',
          onError: 'assetFailure',
        },
      },
      releaseSelection: {
        on: {
          LOCAL: 'selectedLocal',
          FETCH: 'selectedRemote',
          DISCONNECTED: 'initial',
        }
      },
      selectedLocal: {
        invoke: {
          src: 'verifyLocalAsset',
          onDone: 'ready',
          onError: 'assetFailure',
        },
      },
      selectedRemote: {
        invoke: {
          src: 'fetchAsset',
          onDone: 'ready',
          onError: 'assetFailure',
        },
      },
      assetFailure: {

      },
      ready: {
        on: {
          INSTALL: 'installing',
          DISCONNECTED: 'initial',
        }
      },
      installing: {
        invoke: {
          src: 'installAsset',
          onDone: 'installSuccess',
          onError: 'installFailure'
        },
      },
      installSuccess: {
        type: 'final',
        on: {
          DISCONNECTED: 'initial',
        }
      },
      installFailure: {
        type: 'final',
        on: {
          DISCONNECTED: 'initial',
        }
      },
    }
  }, {
    actions: {

    }
  })

  // listen for devices that have arrived
  useEffect(() => {
    const getInitialDevices = async () => {
      let devices = await invoke('fetch_releases')
    }
    const deviceConnected = async () => {
      await listen<ConnectedDevice>('device_connected', event => {
        setDevices(current => [...current, event.payload])
        // route depending on device
        if (event.payload.device_type != null) {
          // all values from ConnectedDeviceType typedef
          switch (event.payload.device_type) {
            case 'Bridge4':
            case 'Bridge6':
              router.push('/bridge')
            case 'BridgeBootloader':
              router.push('/install')
            case 'Click':
              router.push('/click')
            case 'ULoop':
              router.push('/uloop')
            case 'RPBootloader':
              break
          }
        }
      })
    }
    const deviceDisconnected = async () => {
      await listen<ConnectedDevice>('device_disconnected', event => {
        setDevices(current => current.filter((value) => value.id !== event.payload.id))
        router.push('/')
      })
    }
    deviceConnected().catch(console.error)
    deviceDisconnected().catch(console.error)
  }, [router])

  return (
    <div className='m-0 pt-[20vh] h-full flex flex-col justify-between container mx-auto text-center'>
      <div>
        <h2 className='text-xl font-semibold'>Connect one of the following devices to get started:</h2>
        <div className='flex justify-center'>
          <span className='clickable-image'>
            <a href='https://piratemidi.com/pages/bridge-6' target='_blank'>
              <Image
                width={288}
                height={288}
                src={bridge6Image}
                className='!p-4 image-lift'
                alt='Bridge6 Image'
              />
            </a>
          </span>
          <span className='clickable-image'>
            <a href='https://piratemidi.com/pages/bridge4' target='_blank'>
              <Image
                width={288}
                height={288}
                src={bridge4Image}
                className='!p-4 image-lift'
                alt='Bridge4 Image'
              />
            </a>
          </span>
          <span className='clickable-image'>
            <a href='https://piratemidi.com/products/click-midi-interface-relay-switcher' target='_blank'>
              <Image
                width={288}
                height={288}
                src={clickImage}
                className='!p-4 image-lift'
                alt='CLiCK Image'
              />
            </a>
          </span>
          {/* <span className='clickable-image'>
            <a href='https://piratemidi.com/products/%C2%B5loop-4-ch-bypass-and-midi-interface' target='_blank'>
              <Image
                width={288}
                height={288}
                src={uloopImage}
                className='!p-4 image-lift'
                alt='uLOOP Image'
              />
            </a>
          </span> */}
        </div>

        <p>Click a device to learn more about Pirate MIDI's products.</p>
      </div>

      <span className='clickable-image'>
        <a href='https://piratemidi.com/' target='_blank'>
          <Image
            width={75}
            height={75}
            className='logo'
            src={pirateMidiImage}
            alt='Pirate MIDI Logo'
          />
        </a>
      </span>
    </div>
  )
}

export default App;
