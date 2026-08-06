/**
 * Event names for tracking
 */
export const ANALYTICS_EVENTS = {
  // App Lifecycle
  APP_STARTED: "app_started",
  // License Events
  GET_LICENSE: "get_license",
} as const;

/**
 * Capture an analytics event
 */
export const captureEvent = async (
  _eventName: string,
  _properties?: Record<string, any>
) => {
  // Analytics are disabled in this build.
  return;
};

/**
 * Track app initialization
 */
export const trackAppStart = async (
  _appVersion: string,
  _instanceId: string
) => {
  // Analytics are disabled in this build.
  return;
};