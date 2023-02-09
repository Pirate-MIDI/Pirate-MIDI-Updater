import React from "react"
import ContentLoader from "react-content-loader"

const Placeholder = (props) => (
    <ContentLoader
        speed={2}
        width={800}
        height={600}
        viewBox="0 0 800 600"
        backgroundColor="#e2e8f0"
        foregroundColor="#cbd5e1"
        {...props}
    >
        <rect x="196" y="15" rx="5" ry="5" width="400" height="10" />
        <rect x="197" y="45" rx="5" ry="5" width="400" height="10" />
        <rect x="198" y="75" rx="5" ry="5" width="400" height="10" />
        <rect x="15" y="17" rx="0" ry="0" width="161" height="42" />
        <rect x="15" y="75" rx="0" ry="0" width="161" height="42" />
        <rect x="15" y="133" rx="0" ry="0" width="161" height="42" />
        <rect x="15" y="197" rx="0" ry="0" width="161" height="42" />
    </ContentLoader>
)

export default Placeholder

