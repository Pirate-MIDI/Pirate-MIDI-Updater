import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import ReleaseList from "../../components/ReleaseList";

function Bridge() {
    const [releases, setReleases] = useState([]);

    // grab github releases
    useEffect(() => {
        const retrieveReleases = async () => {
            setReleases(await invoke("fetch_releases")) // { device: "bridge6" }
        };
        retrieveReleases();
    }, [setReleases]);

    return (
        <div className="flex">
            <ReleaseList releases={releases} />
            <div>
                <h1>AHOY!</h1>
                <h2>Bridge Connected!</h2>
            </div>
        </div>
    )
}

export default Bridge;