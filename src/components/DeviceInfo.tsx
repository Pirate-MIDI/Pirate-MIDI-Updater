import Image from "next/image";

import bridge6ImageLight from "../assets/bridge6-light.svg"
import bridge6ImageDark from "../assets/bridge6-dark.svg"
import bridge4ImageLight from "../assets/bridge4-light.svg"
import bridge4ImageDark from "../assets/bridge4-dark.svg"

function DeviceInfo(device) {

    let DisplayImage = bridge6ImageLight;
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        DisplayImage = bridge6ImageDark;
    }

    return (
        <div className="flex items-center justify-between p-4 m-2 border-b border-slate-600">
            <div>
                <p>CONNECTED - BRIDGE6</p>
                <p>UID: 000000000000000</p>
            </div>
            <Image
                width={90}
                height={50}
                src={DisplayImage}
                className="w-12"
                alt="Device Image"
            />
        </div>
    )
}

export default DeviceInfo;