import React from "react"
import FadeIn from "react-fade-in/lib/FadeIn"
import BarLoader from "react-spinners/BarLoader"
import ContentLoader from "react-content-loader"

const Placeholder = (props) => (
    <FadeIn className="flex items-center justify-center w-screen h-screen">
        <BarLoader color="#64748B" width={400} />
    </FadeIn>
)

export default Placeholder

