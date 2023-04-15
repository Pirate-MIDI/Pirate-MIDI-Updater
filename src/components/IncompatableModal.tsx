import { Fragment } from 'react'
import { Dialog, Transition } from '@headlessui/react'
import { Bars2Icon, ExclamationTriangleIcon } from '@heroicons/react/24/outline'

function IncompatableModal({ show, onClose, onAccept, device }) {
    let details = device && device.device_details ? device.device_details.hardwareVersion : '1.0.0'
    let head = details.slice(0, 4)
    let tail = details.slice(-1)

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
                            <Dialog.Panel className="w-full max-w-xl p-4 overflow-hidden text-left align-middle transition-all transform bg-white rounded-lg shadow-xl text-slate-800">
                                <Dialog.Title
                                    as="h3"
                                    className="py-2 text-lg font-medium leading-6 border-b text-slate-900"
                                >
                                    <ExclamationTriangleIcon className='inline w-6 h-6 mr-2' />
                                    Incompatable Binary
                                </Dialog.Title>
                                <p className="pb-4 mt-4 text-sm font-bold text-center border-b">
                                    This binary is not compatable with your device!
                                </p>
                                <p className='mt-4 text-sm text-center'>
                                    When manually installing the firmware, the <strong>last</strong> digit of the hardware version of your device needs to match the <strong>last</strong> digit of the binary file version.
                                </p>
                                <div className='flex items-center p-8 justify-evenly'>
                                    <div className='flex flex-col items-center'>
                                        <p className='text-xs uppercase'>Your Device's Hardware Version</p>
                                        <p className='text-lg'>
                                            <span className='text-slate-400'>v{head}</span><strong>{tail}</strong>
                                        </p>
                                    </div>
                                    <Bars2Icon className='w-8 h-8 m-2' />
                                    <div className='flex flex-col items-center'>
                                        <p className='text-xs uppercase'>Example File Name</p>
                                        <p className='text-lg'>
                                            <span className='text-slate-400'>bridgeX_v0.0.0.</span><strong>{tail}</strong><span className='text-lg text-slate-400'>.bin</span>
                                        </p>
                                    </div>
                                </div>
                                <p className="pt-4 mt-2 text-sm text-center border-t">
                                    <strong>Moving forward with installing this firmware file will likely corrupt your device.</strong><br />
                                    Please download the correct version and try again.
                                </p>
                                <div className="flex justify-end">
                                    <button
                                        type="button"
                                        className="inline-flex justify-center px-4 py-2 text-sm font-medium border rounded-md border-pm-blue-left hover:bg-pm-blue-right text-slate-900"
                                        onClick={onAccept}>
                                        Close
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

export default IncompatableModal