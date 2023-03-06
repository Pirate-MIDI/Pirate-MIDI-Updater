import Image from 'next/image';
import FadeIn from "react-fade-in";

import uloopIcon from '../assets/icon-uloop.png'
import clickIcon from '../assets/icon-click.png'
import bridge4Icon from '../assets/icon-bridge4.png'
import bridge6Icon from '../assets/icon-bridge6.png'
import pirateMidiImage from '../assets/logo-piratemidi.png'

function App() {
  return (
    <FadeIn className='w-full h-full overflow-hidden'>
      <div className='mt-[20vh] h-full flex flex-col justify-between container mx-auto text-center'>
        <div>
          <h2 className='text-xl font-semibold'>Connect one of the following devices to get started:</h2>
          <div className='flex justify-center'>
            <span className='clickable-image'>
              <a href='https://piratemidi.com/pages/bridge-6' target='_blank' rel="noreferrer">
                <Image
                  width={835}
                  height={500}
                  src={bridge6Icon}
                  className='!p-4 image-lift'
                  alt='Bridge6 Image'
                />
              </a>
            </span>
            <span className='clickable-image'>
              <a href='https://piratemidi.com/pages/bridge4' target='_blank' rel="noreferrer">
                <Image
                  width={835}
                  height={500}
                  src={bridge4Icon}
                  className='!p-4 image-lift'
                  alt='Bridge4 Image'
                />
              </a>
            </span>
            <span className='clickable-image'>
              <a href='https://piratemidi.com/products/click-midi-interface-relay-switcher' target='_blank' rel="noreferrer">
                <Image
                  width={835}
                  height={500}
                  src={clickIcon}
                  className='!p-4 image-lift'
                  alt='CLiCK Image'
                />
              </a>
            </span>
            {/* <span className='clickable-image'>
              <a href='https://piratemidi.com/products/%C2%B5loop-4-ch-bypass-and-midi-interface' target='_blank' rel="noreferrer">
                <Image
                  width={835}
                  height={500}
                  src={uloopIcon}
                  className='!p-4 image-lift'
                  alt='uLOOP Image'
                />
              </a>
            </span> */}
          </div>

          <p>Click a device to learn more about Pirate MIDI&apos;s products.</p>
        </div>

        <span className='mt-[20vh]'>
          <Image
            width={200}
            height={133}
            src={pirateMidiImage}
            alt='Pirate MIDI Logo'
          />
        </span>
      </div>
    </FadeIn >
  )
}

export default App;
