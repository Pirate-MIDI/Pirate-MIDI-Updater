import Image from 'next/image';
import FadeIn from "react-fade-in";

import ProgressBar from '../components/ProgressBar';

import uloopIcon from '../assets/icon-uloop.png'
import clickIcon from '../assets/icon-click.png'
import bridge4Icon from '../assets/icon-bridge4.png'
import bridge6Icon from '../assets/icon-bridge6.png'
import pirateMidiImage from '../assets/logo-piratemidi.png'

function App() {
  return (
    <FadeIn className='w-full h-screen overflow-hidden' childClassName='h-2/6'>
      <div className='flex justify-end'>
        <Image
          width={125}
          height={133}
          src={pirateMidiImage}
          alt='Pirate MIDI Logo'
        />
      </div>
      <div className='flex flex-col items-center'>
        <h2 className='text-3xl font-semibold'>Connect your device to get started</h2>
        <span className='m-4'>
          <ProgressBar size={75} progress={50} label={'Listening...'} spinnerMode={true} trackWidth={5} indicatorWidth={5} />
        </span>
      </div>
      <div className='flex flex-col items-center bg-gray-300 dark:bg-gray-700'>
        <p className='absolute pt-5 text-sm italic'>Click a device to learn more about Pirate MIDI&apos;s products.</p>
        <div className='flex justify-between pt-3'>
          <span className='clickable-image'>
            <a href='https://piratemidi.com/pages/bridge-6' target='_blank' rel="noreferrer">
              <Image
                width={400}
                height={300}
                src={bridge6Icon}
                className='!p-12 image-lift'
                alt='Bridge6 Image'
              />
            </a>
          </span>
          <span className='clickable-image'>
            <a href='https://piratemidi.com/pages/bridge4' target='_blank' rel="noreferrer">
              <Image
                width={400}
                height={300}
                src={bridge4Icon}
                className='!p-12 image-lift'
                alt='Bridge4 Image'
              />
            </a>
          </span>
          <span className='clickable-image'>
            <a href='https://piratemidi.com/products/click-midi-interface-relay-switcher' target='_blank' rel="noreferrer">
              <Image
                width={400}
                height={300}
                src={clickIcon}
                className='!p-12 image-lift'
                alt='CLiCK Image'
              />
            </a>
          </span>
          <span className='clickable-image'>
            <a href='https://piratemidi.com/products/%C2%B5loop-4-ch-bypass-and-midi-interface' target='_blank' rel="noreferrer">
              <Image
                width={400}
                height={300}
                src={uloopIcon}
                className='!p-12 image-lift'
                alt='uLOOP Image'
              />
            </a>
          </span>
        </div>
      </div>
    </FadeIn >
  )
}

export default App;
