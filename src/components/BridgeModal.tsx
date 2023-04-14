import { Fragment } from 'react'
import { Dialog, Transition } from '@headlessui/react'
import Image from 'next/image'

import bridgeFlexiDiagram from '../assets/diagram-flexi-bridge.png'

function BridgeModal({ show, onClose, onAccept }) {
    return (
        <Transition appear show={show} as={Fragment}>
            <Dialog as="div" className="relative z-10" onClose={onClose}>
                <Transition.Child
                    as={Fragment}
                    enter="ease-out duration-300"
                    enterFrom="opacity-0"
                    enterTo="opacity-100"
                    leave="ease-in duration-200"
                    leaveFrom="opacity-100"
                    leaveTo="opacity-0">
                    <div className="fixed inset-0 bg-black bg-opacity-50" />
                </Transition.Child>

                <div className="fixed inset-0 overflow-y-auto">
                    <div className="flex items-center justify-center min-h-full p-2 text-center">
                        <Transition.Child
                            as={Fragment}
                            enter="ease-out duration-300"
                            enterFrom="opacity-0 scale-95"
                            enterTo="opacity-100 scale-100"
                            leave="ease-in duration-200"
                            leaveFrom="opacity-100 scale-100"
                            leaveTo="opacity-0 scale-95">
                            <Dialog.Panel className="w-full max-w-md p-4 overflow-hidden text-left align-middle transition-all transform bg-white rounded-lg shadow-xl">
                                <Dialog.Title
                                    as="h3"
                                    className="text-lg font-medium leading-6 text-slate-900"
                                >
                                    Issues entering Bootloader
                                </Dialog.Title>
                                <p className="mt-2 text-sm text-slate-500">
                                    Before installing firmware, Bridge devices may occasionally require connecting the <strong>Flexi 1</strong> port to the <strong>Flexi 2</strong> port with a <strong>TS or TRS</strong> cable.
                                </p>
                                <div className='flex justify-center w-full'>
                                    <Image
                                        width={400}
                                        height={294}
                                        src={bridgeFlexiDiagram}
                                        alt={'Bridge Flexi Diagram'}
                                    />
                                </div>
                                <p className="mt-2 text-sm text-slate-500">
                                    Please connect a cable in the configuration shown and start the installation process over again.<br />
                                </p>
                                <p className="mt-4 text-sm font-bold text-center text-slate-500">It is safe to disconnect your device.</p>
                                <div className="flex justify-end mt-4">
                                    <button
                                        type="button"
                                        className="inline-flex justify-center px-4 py-2 text-sm font-medium border rounded-md border-pm-blue-left hover:bg-pm-blue-right text-slate-900"
                                        onClick={onAccept}>
                                        Start Over
                                    </button>
                                </div>
                            </Dialog.Panel>
                        </Transition.Child>
                    </div>
                </div>
            </Dialog>
        </Transition>
    )
}

export default BridgeModal