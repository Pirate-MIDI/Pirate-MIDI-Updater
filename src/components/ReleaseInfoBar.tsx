import ReactMarkdown from 'react-markdown'
import { Disclosure } from '@headlessui/react'
import { ChevronUpIcon, ExclamationTriangleIcon, BookOpenIcon, BookmarkIcon } from '@heroicons/react/24/outline'
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
        if (release && device && release.name && device.device_details) {
            const releaseVer = parseSemVer(release.name)
            const deviceVer = parseSemVer(device.device_details.firmwareVersion)

            return releaseVer.major !== deviceVer.major || releaseVer.minor !== deviceVer.minor
        }
        return false
    }

    const published = release.published_at ? new Date(release.published_at).toDateString() : 'N/A'

    return device && release.body ? (
        <div className='mx-2 h-5/6'>
            <div className='flex items-center justify-between p-4 border-b h-1/6 border-slate-300'>
                <span className='flex items-center'>
                    <span className='text-lg font-bold'>{release.name}</span>
                    <span className={getResetRequired() ? 'flex items-center px-2 py-1 ml-4 text-xs text-yellow-300 border border-yellow-300 rounded' : 'hidden'}>
                        <ExclamationTriangleIcon className='w-4 h-4 mr-2' />
                        <span>Installing this version may require a Factory Reset</span>
                    </span>
                </span>
                <div className='text-right'>
                    <p className='text-xs text-slate-400'>Channel: <strong>{getChannel(release.prerelease)}</strong></p>
                    <p className='text-xs text-slate-400'>Published: <strong>{published}</strong></p>
                </div>
            </div>
            <div className='p-4 overflow-y-auto h-5/6 dark:[color-scheme:dark]'>
                <Disclosure as="div" className={getResetRequired() ? '' : 'hidden'}>
                    {({ open }) => (
                        <>
                            <Disclosure.Button className="flex justify-between w-full px-4 py-2 text-sm font-medium text-left border rounded focus:outline-none focus-visible:ring">
                                <div className='flex items-center'>
                                    <BookOpenIcon className='icon-left' />
                                    <span>Show/Hide Factory Reset Instructions</span>
                                </div>
                                <ChevronUpIcon
                                    className={`${open ? 'rotate-180 transform' : ''
                                        } h-5 w-5`}
                                />
                            </Disclosure.Button>
                            <Disclosure.Panel className="p-4 text-sm">
                                <FadeIn>
                                    <div className='flex items-center px-2 py-1 text-xs text-yellow-300 border border-yellow-300 rounded'>
                                        <ExclamationTriangleIcon className='w-4 h-4 mr-2' />
                                        <p>Performing this action will <strong>delete all presets and user data</strong> from the device.<br />
                                            Utilize the&nbsp;
                                            <a className='inline text-blue-400 underline' href="https://edit.piratemidi.com" target='_blank' rel="noreferrer">
                                                Web Editor
                                            </a>
                                            &nbsp;to create a backup before proceeding.
                                        </p>
                                    </div>
                                    <p className='mt-4'>
                                        <span></span>To Factory Reset your device, do one of the following:<br />
                                        - If you are able to access the device menu on your device, you can find this option under:<br />
                                        &nbsp; &nbsp; &nbsp;- System &gt; Reset &gt; Factory<br />
                                        - If you are unable to access the device menu on your device:<br />
                                        &nbsp; &nbsp; &nbsp;- Hold <strong>Footswitch 1</strong> while applying power to your device<br />
                                    </p>
                                </FadeIn>
                            </Disclosure.Panel>
                        </>
                    )}
                </Disclosure>
                <Disclosure as="div" className={getResetRequired() ? "mt-2" : ''} >
                    {({ open }) => (
                        <>
                            <Disclosure.Button className="flex justify-between w-full px-4 py-2 text-sm font-medium text-left border rounded focus:outline-none">
                                <div className='flex items-center'>
                                    <BookOpenIcon className='icon-left' />
                                    <span>Show/Hide Release Notes</span>
                                </div>
                                <ChevronUpIcon
                                    className={`${open ? 'rotate-180 transform' : ''
                                        } h-5 w-5`}
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
        </div>
    ) : (
        <div className='flex items-center justify-center py-12 text-slate-400'>
            <span>This release does not contain any additional information</span>
        </div>
    )
}

export default ReleaseInfoBar;