import { useEffect, useRef } from "react";
import PropTypes from "prop-types";

const useResizeObserver = ({ callback, element }: { callback: ResizeObserverCallback, element: any }) => {

  const current = element?.current;
  const observer = useRef<any>(null);

  useEffect(() => {

    const observe = () => {
      if (element?.current && observer?.current) {
        observer.current.observe(element.current);
      }
    };

    // if we are already observing old element
    if (observer?.current && current) {
      observer.current.unobserve(current);
    }
    observer.current = new ResizeObserver(callback);
    observe();

    return () => {
      if (observer?.current && current) {
        observer.current.unobserve(current);
      }
    };
  }, [current, callback, element]);
};

useResizeObserver.propTypes = {
  element: PropTypes.object,
  callback: PropTypes.func,
};

export default useResizeObserver;