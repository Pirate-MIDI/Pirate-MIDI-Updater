import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import ReleaseList from "../../components/ReleaseList";
import DeviceInfo from "../../components/DeviceInfo";

function Bridge() {
    const [releases, setReleases] = useState([])
    const [selected, setSelected] = useState(undefined)

    // Retrieve releases from Github and select the latest release available
    useEffect(() => {
        const retrieveReleases = async () => {
            let fetched: any[] = await invoke("fetch_releases") // { device: "bridge6" }
            setReleases(fetched)
            setSelected(fetched[0])
        };
        retrieveReleases()
    }, [])

    return (
        <div className="flex">
            <ReleaseList releases={releases} selected={selected} onSelect={(release) => setSelected(release)} />
            <div className="w-3/4">
                <DeviceInfo />
                <h2>Bridge Connected!</h2>
            </div>
        </div >
    )
}

export default Bridge;