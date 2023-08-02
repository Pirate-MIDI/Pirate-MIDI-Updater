import ReactMarkdown from 'react-markdown'
import { Disclosure } from '@headlessui/react'
import { ChevronUpIcon, ExclamationTriangleIcon, BookOpenIcon } from '@heroicons/react/24/outline'
import FadeIn from 'react-fade-in/lib/FadeIn'
import { parseSemVer } from 'semver-parser'
import { ConnectedDevice } from '../../src-tauri/bindings/ConnectedDevice'
import { Release } from '../../src-tauri/bindings/Release'

function ReleaseInfoBar({ device, release }: { device: ConnectedDevice, release: Release }) {
    const getChannel = (prerelease) => {
        return prerelease ? (
            <span className='text-amber-500'>Beta</span>
        ) : (
            <span className='text-emerald-500'>Stable</span>
        )
    }

    const getResetRequired = () => {
        console.log("determineing reset")
        if (release && device && release.name && device.device_details
            && (device.device_type === 'Bridge4' || device.device_type === 'Bridge6')) {
            const releaseVer = parseSemVer(release.name)
            const deviceVer = parseSemVer(device.device_details.firmwareVersion)

            return releaseVer.major !== deviceVer.major || releaseVer.minor !== deviceVer.minor
        }
        return false
    }

    const published = release.published_at ? new Date(release.published_at).toDateString() : 'N/A'

    const getDescription = (release) => {
        return release && release.body ? (
            <div className='p-4 overflow-y-auto h-5/6 dark:[color-scheme:dark]'>
                <Disclosure as="div" className={getResetRequired() ? '' : 'hidden'}>
                    {({ open }) => (
                        <>
                            <Disclosure.Button className="flex justify-between w-full px-4 py-2 text-sm font-medium text-left text-yellow-600 border border-yellow-600 rounded dark:text-yellow-300 dark:border-yellow-300">
                                <div className='flex items-center'>
                                    <BookOpenIcon className='icon-left' />
                                    <span>Show/Hide Factory Reset Instructions</span>
                                </div>
                                <ChevronUpIcon
                                    className={`${open ? 'rotate-180 transform' : ''
                                        } h-5 w-5 text-yellow-600 dark:text-yellow-300`}
                                />
                            </Disclosure.Button>
                            <Disclosure.Panel className="p-4 text-sm">
                                <FadeIn>
                                    <div className='flex items-center px-2 py-1 text-xs border rounded border-pm-red-right text-pm-red-right'>
                                        <ExclamationTriangleIcon className='w-4 h-4 mr-2' />
                                        <p><strong>Performing this action will delete all presets and user data from the device.</strong><br />
                                            Utilize the&nbsp;
                                            <a className='inline text-blue-400 underline' href="https://pirate-midi-dev.web.app/backup" target='_blank' rel='noreferrer'>
                                                Backup Utility
                                            </a>
                                            &nbsp;or&nbsp;
                                            <a className='inline text-blue-400 underline' href="https://edit.piratemidi.com" target='_blank' rel="noreferrer">
                                                Web Editor
                                            </a>
                                            &nbsp;to create a backup before proceeding.
                                        </p>
                                    </div>
                                    <p className='mt-4'>
                                        To Factory Reset Bridge devices:<br />
                                        - Hold <strong>Footswitch 1</strong> down while applying power to your device. Power can be applied via 9v or USB. Hold until all LEDs have flashed white.
                                    </p>
                                </FadeIn>
                            </Disclosure.Panel>
                        </>
                    )}
                </Disclosure>
                <Disclosure as="div" defaultOpen={true} className={getResetRequired() ? "mt-4" : ''} >
                    {({ open }) => (
                        <>
                            <Disclosure.Button className="flex justify-between w-full px-4 py-2 text-sm font-medium text-left border rounded focus:outline-none text-pm-blue-left border-pm-blue-left dark:text-pm-blue-right dark:border-pm-blue-right">
                                <div className='flex items-center'>
                                    <BookOpenIcon className='icon-left' />
                                    <span>Show/Hide Release Notes</span>
                                </div>
                                <ChevronUpIcon
                                    className={`${open ? 'rotate-180 transform' : ''
                                        } h-5 w-5 text-pm-blue-left dark:text-pm-blue-right`}
                                />
                            </Disclosure.Button>
                            <Disclosure.Panel>
                                <FadeIn>
                                    <ReactMarkdown className='text-sm markdown'>{release.body}</ReactMarkdown>
                                </FadeIn>
                            </Disclosure.Panel>

                        </>
                    )}
                </Disclosure>
            </div>
        ) : (
            <div className='flex items-center justify-center py-12 text-slate-400'>
                <span>This release does not contain any additional information</span>
            </div>
        )
    }

    return device ? (
        <div className='mx-2 h-5/6'>
            <div className='flex items-center justify-between p-4 border-b h-1/6 border-slate-300'>
                <span className='flex items-center'>
                    <span className='text-lg font-bold'>{release.name}</span>
                    <span className={getResetRequired() ? 'flex items-center px-2 py-1 ml-4 text-xs font-bold text-yellow-600 border border-yellow-600 dark:text-yellow-300 dark:border-yellow-300 rounded' : 'hidden'}>
                        <ExclamationTriangleIcon className='w-4 h-4 mr-2' />
                        <span>Installing this version may require a Factory Reset</span>
                    </span>
                </span>
                <div className='text-right'>
                    <p className='text-xs text-slate-400'>Channel: <strong>{getChannel(release.prerelease)}</strong></p>
                    <p className='text-xs text-slate-400'>Published: <strong>{published}</strong></p>
                </div>
            </div>
            {getDescription(release)}
        </div>
    ) : (
        <div className='flex items-center justify-center py-12 text-slate-400'>
            <span>This release does not contain any additional information</span>
        </div>
    )
}

export default ReleaseInfoBar;