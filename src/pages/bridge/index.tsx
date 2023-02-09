import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import ReleaseList from "../../components/ReleaseListColumn";
import DeviceInfo from "../../components/DeviceInfoBar";
import Placeholder from "../../components/Placeholder";
import FadeIn from "react-fade-in";
import ReleaseInfo from "../../components/ReleaseInfoBar";
import InstallBar from "../../components/InstallBar";

function Bridge() {
    const [spinner, setSpinner] = useState(true);
    const [releases, setReleases] = useState([])
    const [selected, setSelected] = useState(undefined)

    // Retrieve releases from Github and select the latest release available
    useEffect(() => {
        const retrieveReleases = async () => {
            let fetched: any[] = await invoke("fetch_releases") // { device: "bridge6" }
            setReleases(fetched)
            setSelected(fetched[0])
            setSpinner(false)
        };
        retrieveReleases()
    }, [])

    return spinner ? (
        <Placeholder />
    ) : (
        <FadeIn>
            <div className="flex h-screen overflow-hidden">
                <ReleaseList releases={releases} selected={selected} onSelect={(release) => setSelected(release)} />
                <div className="w-3/4">
                    <DeviceInfo />
                    <ReleaseInfo release={selected} />
                    <InstallBar release={selected} />
                </div>
            </div>
        </FadeIn>
    )
}

export default Bridge;