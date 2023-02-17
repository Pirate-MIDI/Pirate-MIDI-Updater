import { ConnectedDevice } from "../../src-tauri/bindings/ConnectedDevice"

import bridge6Light from '../assets/logo-bridge6-light.svg'
import bridge6Dark from '../assets/logo-bridge6-dark.svg'
import bridge4Light from '../assets/logo-bridge4-light.svg'
import bridge4Dark from '../assets/logo-bridge4-dark.svg'
import clickDark from '../assets/logo-click-dark.svg'
import clickLight from '../assets/logo-click-light.svg'
import uLoopDark from '../assets/logo-uloop-dark.svg'
import uLoopLight from '../assets/logo-uloop-light.svg'

function DeviceLogo(device: ConnectedDevice) {
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        switch (device.device_type) {
            case "Bridge6":
                return bridge6Dark
            case "Bridge4":
                return bridge4Dark
            case "Click":
                return clickDark
            case "ULoop":
                return uLoopDark
        }
    } else {
        switch (device.device_type) {
            case "Bridge6":
                return bridge6Light
            case "Bridge4":
                return bridge4Light
            case "Click":
                return clickLight
            case "ULoop":
                return uLoopLight
        }
    }
}

export default DeviceLogo